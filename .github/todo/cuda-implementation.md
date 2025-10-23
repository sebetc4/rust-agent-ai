# CUDA Implementation Guide

## Overview

This document outlines the steps needed to enable CUDA GPU acceleration in the agents-rs application using llama-cpp-2.

## Current Status

- ✅ GPU infrastructure implemented in Rust backend
- ✅ LLMConfig enhanced with GPU parameters (use_gpu, n_gpu_layers, main_gpu)
- ✅ LLMEngine with GPU detection and configuration methods
- ✅ Tauri commands for GPU management (get_gpu_info, detect_gpu, update_gpu_settings)
- ⏳ CUDA features temporarily disabled due to missing CUDA toolkit
- ⏳ Frontend GPU controls not yet implemented

## Prerequisites

### System Requirements

**For NVIDIA CUDA:**
```bash
# Check if NVIDIA GPU is available
nvidia-smi

# Install CUDA toolkit (Fedora)
sudo dnf install cuda-toolkit cuda-devel

# Verify CUDA installation
nvcc --version

# Check CUDA compute capability
nvidia-smi --query-gpu=compute_cap --format=csv
```

**For Apple Metal (macOS only):**
- macOS 10.13+ with Metal-compatible GPU
- Xcode command line tools installed

**For OpenCL (cross-platform fallback):**
```bash
# Fedora
sudo dnf install opencl-headers ocl-icd-devel

# Check OpenCL devices
clinfo
```

## Implementation Steps

### 1. Enable GPU Features in Cargo.toml

Currently disabled features need to be re-enabled:

```toml
[dependencies]
llama-cpp-2 = { version = "0.1.122", features = ["cuda", "metal"] }

[features]
default = ["gpu"]
gpu = ["cuda", "metal"]
cuda = []
metal = []
cpu-only = []
```

### 2. Environment Variables

Set appropriate environment variables for compilation:

```bash
# For CUDA
export CUDA_PATH=/usr/local/cuda
export LD_LIBRARY_PATH=$CUDA_PATH/lib64:$LD_LIBRARY_PATH
export PATH=$CUDA_PATH/bin:$PATH

# For compilation with specific CUDA architecture
export CUDA_COMPUTE_CAP=75  # Adjust based on your GPU
```

### 3. Compilation Fixes

Address the current compilation issues:

**Issue 1: CUDA toolkit not found**
```bash
# Verify CUDA installation
which nvcc
ls -la /usr/local/cuda/lib64/
```

**Issue 2: llama-cpp-sys build failures**
- Ensure cmake and clang are installed
- Set proper CUDA_PATH environment variable
- May need to rebuild llama-cpp-2 with GPU support

### 4. Code Changes Required

**LLMEngine enhancements:**
```rust
// In src-tauri/src/llm/engine.rs
impl LLMEngine {
    pub fn detect_gpu_config(&self) -> Result<GPUInfo> {
        // Implement actual GPU detection
        // Currently returns mock data
    }
    
    pub fn load_model_with_gpu(&mut self, config: &LLMConfig) -> Result<()> {
        // Apply GPU parameters to llama-cpp-2 context
        let params = llama_cpp_2::context::LlamaContextParams::default()
            .with_n_gpu_layers(config.n_gpu_layers)
            .with_main_gpu(config.main_gpu);
        
        // Load model with GPU parameters
    }
}
```

### 5. Frontend Integration

Create UI components for GPU configuration:

**Components needed:**
- `GPUSettings.tsx` - GPU configuration panel
- `GPUStatus.tsx` - GPU status indicator
- `ModelSelector.tsx` - Model selection with GPU options

**State management:**
```typescript
// In stores/gpu.ts
interface GPUStore {
  isGPUEnabled: boolean;
  gpuLayers: number;
  mainGPU: number;
  gpuInfo: GPUInfo | null;
  updateGPUSettings: (settings: GPUSettings) => void;
  detectGPU: () => Promise<void>;
}
```

## Testing Plan

### 1. GPU Detection Test
```bash
cd src-tauri
cargo run --example test_gpu_detection
```

### 2. Model Loading Test
```bash
# Test with GPU enabled
cargo run --example test_model_gpu

# Compare performance with CPU
cargo run --example test_model_cpu
```

### 3. Integration Test
```bash
# Run full application with GPU
pnpm tauri dev

# Test GPU settings in UI
# Verify GPU memory usage with nvidia-smi
```

## Performance Expectations

**Expected improvements with CUDA:**
- 5-20x faster inference depending on model size
- Lower CPU usage
- Better throughput for larger models (7B+ parameters)

**Optimal settings:**
- For 8GB VRAM: n_gpu_layers = 20-25 (for 7B models)
- For 12GB VRAM: n_gpu_layers = 35-40 (for 13B models)
- For 24GB VRAM: Full model offload possible

## Troubleshooting

### Common Issues

**1. "CUDA not found" during compilation**
```bash
# Solution: Install CUDA toolkit and set environment variables
export CUDA_PATH=/usr/local/cuda
```

**2. "Insufficient GPU memory" at runtime**
```bash
# Solution: Reduce n_gpu_layers or use smaller model
# Monitor with: nvidia-smi -l 1
```

**3. "No CUDA-capable device found"**
```bash
# Check: nvidia-smi
# Verify: NVIDIA drivers installed correctly
```

**4. Performance not improving**
```bash
# Check: GPU utilization with nvidia-smi
# Verify: Model actually loaded on GPU
# Adjust: n_gpu_layers parameter
```

## Alternative: CPU-Only Mode

For systems without GPU support, ensure CPU-only compilation works:

```toml
[features]
default = ["cpu-only"]
cpu-only = []

[dependencies]
llama-cpp-2 = { version = "0.1.122", default-features = false }
```

## Next Steps

1. **Immediate:** Install CUDA toolkit on development machine
2. **Enable:** GPU features in Cargo.toml
3. **Test:** GPU detection and model loading
4. **Implement:** Frontend GPU controls
5. **Optimize:** Performance tuning and memory management
6. **Document:** User guide for GPU setup

## Resources

- [llama-cpp-2 CUDA documentation](https://docs.rs/llama-cpp-2/)
- [NVIDIA CUDA Installation Guide](https://developer.nvidia.com/cuda-downloads)
- [Tauri GPU Integration Best Practices](https://tauri.app/v1/guides/)