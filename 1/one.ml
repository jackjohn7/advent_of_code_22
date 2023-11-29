open Printf

(* Not even remotely finished *)

let data =
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";;

let max l =
  let rec aux lst max =
    match lst with
    | [] -> max
    | h :: tl -> if h >= max then aux tl h else aux tl max
  in
  aux l 0;;

let sum l =
  let rec aux lst acc =
    match lst with
    | [] -> acc
    | h :: tl -> aux tl (acc + h)
  in aux l 0;;

let print_list lst =
  List.iter (fun x -> print_endline (String.trim x)) lst;;

let () =
  print_endline (string_of_int (sum [4; 5; 1;]));
  print_list (String.split_on_char '\n' data);;
