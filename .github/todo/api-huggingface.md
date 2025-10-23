## 🧠 Hugging Face — API for Listing and Downloading Models

### 📍 Main Endpoints

#### 1. **List Models**

* **HTTP Endpoint:**

  ```
  GET https://huggingface.co/api/models
  ```

* **Available query parameters:**

  * `search` → free-text search (e.g. `"bert"`)
  * `author` → model owner or organization
  * `task` → ML task (e.g. `"text-classification"`, `"image-segmentation"`)
  * `library` → framework (e.g. `"pytorch"`, `"tensorflow"`)
  * `language` → language of the model
  * `sort` → sorting key (e.g. `"downloads"`, `"likes"`)
  * `direction` → `"asc"` or `"desc"`
  * `limit`, `full` → pagination and result detail level

* **Example:**

  ```bash
  curl "https://huggingface.co/api/models?search=bert&limit=5"
  ```

* **Response:**

  ```json
  [
    {
      "modelId": "bert-base-uncased",
      "author": "google",
      "downloads": 1234567,
      "likes": 2300,
      "pipeline_tag": "fill-mask",
      "tags": ["pytorch", "transformers"]
    },
    ...
  ]
  ```

---

#### 2. **Download a Model File**

* **HTTP Endpoint:**

  ```
  GET https://huggingface.co/<repo_id>/resolve/<revision>/<filename>
  ```

  * `<repo_id>` → e.g. `bert-base-uncased`
  * `<revision>` → usually `main`, a tag, or a commit SHA
  * `<filename>` → e.g. `config.json`, `pytorch_model.bin`

* **Example:**

  ```bash
  curl -L "https://huggingface.co/bert-base-uncased/resolve/main/config.json" -o config.json
  ```

---

#### 3. **Download the Whole Repository (Snapshot)**

* There’s no single “download all” HTTP endpoint, but you can:

  * Call `/api/models/{repo_id}` to get the list of all files.
  * Then download each file individually.

* **Example (get file list):**

  ```
  GET https://huggingface.co/api/models/bert-base-uncased
  ```

  → the `"siblings"` field contains the file paths and metadata.

---

### 🔐 Authentication

* Some models require a **Hugging Face access token**.
* Add the following header:

  ```
  Authorization: Bearer <HF_TOKEN>
  ```

---

### ⚙️ Technical Notes

* All responses are JSON.
* Pagination via `Link` headers.
* Standard rate limiting applies.
* Works with any HTTP client (e.g., `reqwest`, `surf`, etc. in Rust).

---

### 💡 Suggested Rust Implementation

* Use [`reqwest`](https://docs.rs/reqwest/) or [`ureq`](https://docs.rs/ureq/).
* Suggested function structure:

  ```rust
  fn list_models(query: &str) -> Result<Vec<Model>, Error>;
  fn download_model_file(repo_id: &str, filename: &str) -> Result<Vec<u8>, Error>;
  ```
* Deserialize JSON responses using [`serde`](https://docs.rs/serde/) for convenience.
