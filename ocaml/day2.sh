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


let first_part = function
| "A X" -> 3 + 1
| "A Y" -> 6 + 2
| "A Z" -> 0 + 3
| "B X" -> 0 + 1
| "B Y" -> 3 + 2
| "B Z" -> 6 + 3
| "C X" -> 6 + 1
| "C Y" -> 0 + 2
| "C Z" -> 3 + 3
| _ -> 0

let second_part = function
| "A X" -> 0 + 3
| "A Y" -> 3 + 1
| "A Z" -> 6 + 2
| "B X" -> 0 + 1
| "B Y" -> 3 + 2
| "B Z" -> 6 + 3
| "C X" -> 0 + 2
| "C Y" -> 3 + 3
| "C Z" -> 6 + 1
| _ -> 0

let handle_line line_to_int sum line =
  sum + (line_to_int line)

let score_for_part_one =
  (read_file "../inputs/day2/secret_strategy.txt")
  |> List.fold_left (handle_line first_part) 0;;

let score_for_part_two =
  (read_file "../inputs/day2/secret_strategy.txt")
  |> List.fold_left (handle_line second_part) 0;;

Format.printf "@[Score in first part:%i, for second part : %i  @]@." score_for_part_one score_for_part_two
