fn longest_consec(strarr: Vec<&str>, k: usize) -> String 
{

    if strarr.len() == 0 || k > strarr.len() || k == 0 
    {
        return "".to_string();
    }
   
    strarr.windows(k)
    .map(|win| win.concat())
    .fold(String::new(), |acc, x| 
        {
        if x.len() > acc.len() { x } else { acc }
        })
}
