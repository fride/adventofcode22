#!/usr/bin/env nix-shell
(*
#!nix-shell --pure -i ocaml -p ocaml
*)

type elf = {id : int; calories : int}

let print_elf elf =
 Format.sprintf "@[<1>%i@ :: %i@]@." elf.id elf.calories;;

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

let lines = (read_file "../crates/day1/calories.txt");;

let parse_line line =
try
  int_of_string line
with failure ->
  -1;;

let hanle_line acc line =
try
  let calories = int_of_string line in
  let an_elf = List.hd acc in
    {id =an_elf.id; calories = calories + an_elf.calories} :: List.tl acc
with failure ->
  let an_elf = List.hd acc in
    {id = an_elf.id + 1; calories = 0} :: acc;;

let calories_per_elf = List.fold_left hanle_line [{id = 1; calories = 0}] lines;;

let masters_of_calories =
  let sorted = List.sort
    (fun a b -> compare b.calories a.calories)
    calories_per_elf in
    match sorted with
      | [] -> "No elves to be found!"
      | a :: [] -> print_elf a
      | a :: b :: [] -> String.concat ", " [(print_elf a); (print_elf b)]
      | a :: b :: c ::rest -> String.concat ", " [(print_elf a); (print_elf b); (print_elf c)];;

Format.printf "@[<1>%s@]@." masters_of_calories


