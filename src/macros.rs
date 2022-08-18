macro_rules! endpoint{
    ($first: expr, $second: expr, $third: path)=>{
        endpoint!($first, $second, routes![$third])
    };
    ($first: expr, $second: expr, $third: expr)=>{
        $first = Box::new($first.mount($second, $third))
    };
}