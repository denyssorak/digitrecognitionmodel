Trying to create an AI model that will recognize hand-written digits  
Hope it turns out well and cool  
:)  
  
# Handwritten Digit Recognizer (Rust, no ML libraries)  
  
A neural network built from scratch in Rust — no external crates for the ML  
itself. The goal is to translate known linear algebra / calculus into real  
code, not just call a library.  
  
## Plan  
  
- [x] **1. Data loading** — parse the MNIST IDX binary format by hand  
  - [x] `load_labels`: read magic number + count header, then one byte per label  
  - [x] `load_images`: read magic/count/rows/cols header, then 784 bytes per image  
- [ ] **2. `Matrix` type** — flat `Vec<f64>`, row-major, backing all the math  
  - [x] struct + `new` / `get` / `set`  
  - [x] `add`  
  - [x] `sub`  
  - [x] elementwise `mul` (Hadamard)  
  - [ ] `transpose`  
  - [ ] `matmul`  
  - [ ] `apply` (map a function over every element)  
- [ ] **3. Network structure** — `Layer` (weights + biases) and `Network` (`Vec<Layer>`), forward pass  
- [ ] **4. Activation functions** — sigmoid/ReLU + softmax, plus their derivatives  
- [ ] **5. Loss function** — cross-entropy loss + its gradient  
- [ ] **6. Backpropagation** — manual chain rule, layer by layer  
- [ ] **7. Weight init** — own PRNG (no `rand` crate)  
- [ ] **8. Training loop** — mini-batch SGD, shuffle + batch + update per epoch  
- [ ] **9. Evaluation** — accuracy on the held-out test set  
- [ ] **10. Iterate** — tune learning rate, try more layers/ReLU/momentum/L2  
  
No external dependencies (`Cargo.toml` stays empty under `[dependencies]`).  
