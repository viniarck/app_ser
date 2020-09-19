use rustler::{Encoder, Env, Error, Term};
use serde::{Deserialize, Serialize};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Value {
    value: i32,
}

rustler::rustler_export_nifs! {
    "Elixir.AppSerNif",
    [
        ("add", 2, add),
        ("sum", 1, sum)
    ],
    None
}

fn sum<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let val: &str = args[0].decode()?;
    let array: Vec<Value> = serde_json::from_str(val).unwrap();
    let res = array.iter().fold(0, |sum, i| sum + i.value);
    Ok((atoms::ok(), res).encode(env))
}


fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let num1: i64 = args[0].decode()?;
    let num2: i64 = args[1].decode()?;

    Ok((atoms::ok(), num1 + num2).encode(env))
}
