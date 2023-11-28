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

let print_list lst =
  List.iter (fun x -> print_endline (String.trim x)) lst

let () =
  print_list (String.split_on_char '\n' data);

  print_endline "Hello, world";;
