import torch
import time
import logging
from transformers import AutoTokenizer, AutoModelForCausalLM
from fastapi import FastAPI, HTTPException, BackgroundTasks
from pydantic import BaseModel
import uvicorn
import asyncio
from typing import Optional

# Configure logging
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)

# Create FastAPI app
app = FastAPI(
    title="DeepSeek Coder API",
    description="API for generating code using DeepSeek Coder model",
    version="1.0.0",
)


# Define request model
class GenerationRequest(BaseModel):
    prompt: str
    max_tokens: int = 100  # Changed from max_length to avoid confusion
    temperature: float = 0.7
    timeout: Optional[int] = 120  # Add timeout in seconds


# Load model and tokenizer
logger.info("Loading model and tokenizer...")
try:
    tokenizer = AutoTokenizer.from_pretrained("deepseek-ai/deepseek-coder-1.3b-base")
    model = AutoModelForCausalLM.from_pretrained(
        "deepseek-ai/deepseek-coder-1.3b-base",
        torch_dtype=torch.float16,
        device_map="auto",
        offload_folder="./offload_folder",
        low_cpu_mem_usage=True,
    )

    # Explicitly set pad token if it doesn't exist
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token
        logger.info(f"Set pad_token_id to eos_token_id: {tokenizer.pad_token_id}")

    logger.info("Model loaded successfully!")
except Exception as e:
    logger.error(f"Error loading model: {str(e)}")
    raise

# Track generation status
generation_stats = {
    "total_requests": 0,
    "successful_requests": 0,
    "failed_requests": 0,
    "average_generation_time": 0,
}


def generate_response(prompt, max_tokens=100, temperature=0.7):
    start_time = time.time()
    logger.info(f"Starting generation for prompt: {prompt[:50]}...")

    try:
        # Tokenize input
        logger.info("Tokenizing input...")
        inputs = tokenizer(
            prompt,
            return_tensors="pt",
            padding=True,
            truncation=True,
            max_length=1024,
            return_attention_mask=True,
        ).to(model.device)

        logger.info("Running model.generate()...")

        with torch.no_grad():
            # Use only max_new_tokens to avoid the warning
            outputs = model.generate(
                input_ids=inputs.input_ids,
                attention_mask=inputs.attention_mask,
                max_new_tokens=max_tokens,  # Only use this parameter, not max_length
                temperature=temperature,
                top_p=0.9,
                do_sample=True,
                pad_token_id=tokenizer.pad_token_id,
                num_beams=1,  # Use greedy search for faster generation
            )

        # Get only the generated part (not the input)
        input_length = inputs.input_ids.shape[1]
        generated_tokens = outputs[0][input_length:]

        # Decode only the generated part
        logger.info("Decoding response...")
        response = tokenizer.decode(generated_tokens, skip_special_tokens=True)

        elapsed_time = time.time() - start_time
        logger.info(f"Generation completed in {elapsed_time:.2f} seconds")

        # Update stats
        generation_stats["successful_requests"] += 1
        generation_stats["average_generation_time"] = (
            generation_stats["average_generation_time"]
            * (generation_stats["successful_requests"] - 1)
            + elapsed_time
        ) / generation_stats["successful_requests"]

        return response

    except Exception as e:
        logger.error(f"Error in generate_response: {str(e)}")
        generation_stats["failed_requests"] += 1
        raise


async def generate_with_timeout(prompt, max_tokens, temperature, timeout):
    """Run generation with a timeout"""
    loop = asyncio.get_event_loop()
    try:
        # Run the CPU-intensive generate_response in a thread pool
        return await asyncio.wait_for(
            loop.run_in_executor(
                None, lambda: generate_response(prompt, max_tokens, temperature)
            ),
            timeout=timeout,
        )
    except asyncio.TimeoutError:
        logger.warning(f"Generation timed out after {timeout} seconds")
        generation_stats["failed_requests"] += 1
        raise HTTPException(status_code=504, detail="Generation timed out")


@app.post("/generate", response_model=dict)
async def generate(request: GenerationRequest):
    generation_stats["total_requests"] += 1

    logger.info(f"Received generation request: {request.prompt[:50]}...")

    try:
        # Use the timeout wrapper
        response = await generate_with_timeout(
            request.prompt, request.max_tokens, request.temperature, request.timeout
        )

        return {
            "prompt": request.prompt,
            "response": response,
            "stats": {
                "generation_time_seconds": generation_stats["average_generation_time"]
            },
        }

    except asyncio.TimeoutError:
        raise HTTPException(
            status_code=504,
            detail=f"Generation timed out after {request.timeout} seconds",
        )
    except Exception as e:
        logger.error(f"Error during generation: {str(e)}")
        raise HTTPException(
            status_code=500, detail=f"Error generating response: {str(e)}"
        )


@app.get("/health")
async def health_check():
    return {
        "status": "healthy",
        "model": "deepseek-ai/deepseek-coder-1.3b-base",
        "stats": generation_stats,
    }


if __name__ == "__main__":
    # Start the server manually using uvicorn
    uvicorn_config = uvicorn.Config(
        app=app,
        host="0.0.0.0",
        port=8080,
        log_level="info",
        reload=False,
        workers=1,
        timeout_keep_alive=240,  # Longer keep-alive timeout
    )
    server = uvicorn.Server(uvicorn_config)
    server.run()
