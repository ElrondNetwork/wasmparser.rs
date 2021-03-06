/* Copyright 2017 Mozilla Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// The following limits are updated to fit the restrictions of a WASM smart
// contract, which should be much lower than for the usual WASM use-case.
pub const MAX_WASM_TYPES: usize = 10_000;
pub const MAX_WASM_FUNCTIONS: usize = 10_000;
pub const _MAX_WASM_IMPORTS: usize = 1000;
pub const _MAX_WASM_EXPORTS: usize = 1000;
pub const MAX_WASM_GLOBALS: usize = 4000; // max 32K
pub const _MAX_WASM_DATA_SEGMENTS: usize = 10_000;
pub const MAX_WASM_MEMORY_PAGES: usize = 1000; // max 64M memory
pub const MAX_WASM_STRING_SIZE: usize = 100_000;
pub const _MAX_WASM_MODULE_SIZE: usize = 1024 * 1024 * 32; // 32M
pub const MAX_WASM_FUNCTION_SIZE: usize = 128 * 1024;
pub const MAX_WASM_FUNCTION_LOCALS: usize = 4000;
pub const MAX_WASM_FUNCTION_PARAMS: usize = 100;
pub const MAX_WASM_FUNCTION_RETURNS: usize = 100;
pub const _MAX_WASM_TABLE_SIZE: usize = 10_000;
pub const MAX_WASM_TABLE_ENTRIES: usize = 10_000;
pub const MAX_WASM_TABLES: usize = 1;
pub const MAX_WASM_MEMORIES: usize = 1;
