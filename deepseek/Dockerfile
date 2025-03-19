FROM python:3.10-slim

WORKDIR /deepseek

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements file
COPY requirements.txt .

# Install Python dependencies
RUN pip install --upgrade pip
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY app.py .

# Create directory for model offloading
RUN mkdir -p ./offload_folder

# Expose port
EXPOSE 8080

# Command to run the application
CMD ["python3", "app.py"]