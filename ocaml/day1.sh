#!/usr/bin/env nix-shell
(*
#!nix-shell --pure -i ocaml -p ocaml
*)

let read_file filename =
let lines = ref [] in
let chan = open_in filename in
try
  while true; do
    lines := input_line chan :: !lines
  done; !lines
with End_of_file ->
  close_in chan;
  List.rev !lines ;;

let lines = (read_file "./inputs/day1/calories.txt");;

let parse_line line =

let should_now_work = List.fold_left  int_of_string lines;;

let text = String.concat "\n" (read_file "./inputs/day1/calories.txt");;
Format.printf "@[<1>%s@]@." text;;
