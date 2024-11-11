

# Fibonnaci (Sample Bonsol Program Guide)

## Prerequisites

### Install Flatbuffers
```bash
# Install required dependencies
sudo apt update
sudo apt install -y git cmake build-essential

# Clone and build FlatBuffers
git clone https://github.com/google/flatbuffers.git
cd flatbuffers
git checkout v23.5.26

mkdir build && cd build
cmake ..
make -j$(nproc)
sudo make install

# Verify installation
flatc --version
```

### Install Docker
```bash
# Remove old versions
sudo apt-get remove docker docker-engine docker.io containerd runc

# Install prerequisites
sudo apt-get update
sudo apt-get install -y \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

# Add Docker's GPG key and repository
sudo mkdir -m 0755 -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install Docker
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# Setup Docker permissions
sudo groupadd -f docker
sudo usermod -aG docker $USER
newgrp docker

# Test installation
docker run hello-world
```

### Install Solana and Anchor
```bash
# Install system dependencies
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libudev-dev llvm libclang-dev \
    protobuf-compiler libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
rustup default 1.79.0

# Install Solana
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
agave-install init 1.18.22

# Install Node.js and package managers
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install -g corepack
sudo corepack enable yarn
sudo corepack enable pnpm

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm install 0.30.1
avm use 0.30.1
```

### Install Bonsol CLI
```bash
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
cargo binstall cargo-risczero
cargo risczero install
cargo install bonsol-cli --git https://github.com/anagrambuild/bonsol --features linux
```


Setup the local environment:
```bash
# Generate Solana keypair if needed
solana-keygen new -o ~/.config/solana/id.json

# Configure Solana for local use
solana config set -ul

# Install prover and setup environment
bash ./bin/install_prover.sh
bash ./bin/setup.sh
```

Start the validator (in a new terminal):
```bash
cd bonsol
bash ./bin/validator.sh
```

Start the node (in another terminal):
```bash
bash ./bin/run-node.sh
```



## Troubleshooting

If you encounter any issues:

1. Check that all services are running:
- Validator should be running on port 8899
- Node should be running and processing requests
- Docker daemon should be active

2. Verify your Solana configuration:
```bash
solana config get
```

3. Check your keypair is properly funded for local development:
```bash
solana balance
```

4. If you need to generate a new keypair:
```bash
solana-keygen new -o ~/.config/solana/id.json
```

## Deployment Options

You can deploy your zkprogram in three ways:
- S3 (recommended)
- Manual URL
- ShadowDrive

### Deploy using S3
```bash
bonsol deploy -m ./manifest.json \
  -t s3 \
  --bucket YOUR_BUCKET_NAME \
  --access-key YOUR_ACCESS_KEY \
  --secret-key YOUR_SECRET_KEY \
  --region YOUR_REGION \
  --auto-confirm
```

### Deploy using URL
```bash
bonsol deploy -m ./manifest.json \
  -t url \
  --url YOUR_PUBLIC_URL \
  --auto-confirm
```

### Deploy using ShadowDrive
```bash
bonsol deploy -m ./manifest.json \
  -t shadow-drive \
  --storage-account YOUR_STORAGE_ACCOUNT \
  --auto-confirm
```

## Execute the Program

Create an execution request JSON file (e.g. `execution-request.json`):
```json
{
  "programId": "YOUR_PROGRAM_ID",
  "inputs": {
    "Public": "YOUR_PUBLIC_INPUT",
    "Private": "YOUR_PRIVATE_INPUT"
  }
}
```

Execute the program:
```bash
bonsol execute \
  -f execution-request.json \
  -x 2000 \
  -m 2000 \
  -w
```

Command flags:
- `-x`: Expiry time in slots
- `-m`: Tip amount in lamports
- `-w`: Wait for execution completion

## Important Notes

1. Programs are immutable - any changes require redeployment with a new image ID

2. For local development, you can use the default Solana config

For more detailed information about Bonsol, visit [Bonsol.sh](https://bonsol.sh).