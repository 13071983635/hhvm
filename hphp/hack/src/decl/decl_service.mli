(**
 * Copyright (c) 2015, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the BSD-style license found in the
 * LICENSE file in the "hack" directory of this source tree. An additional grant
 * of patent rights can be found in the PATENTS file in the same directory.
 *
 *)


(*****************************************************************************)
(* Module declaring the types in parallel *)
(*****************************************************************************)

(* The result expected from the service *)
type result = Errors.t

(* Used for lazy typechecking *)
type lazy_decl_result = Errors.t

(*****************************************************************************)
(* Starts the process *)
(*****************************************************************************)
val go: WorkerController.worker list option -> bucket_size:int -> TypecheckerOptions.t ->
  FileInfo.fast -> result
val merge_lazy_decl: lazy_decl_result -> lazy_decl_result -> lazy_decl_result
