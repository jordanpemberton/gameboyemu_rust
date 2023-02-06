/*
add
borrow
carry
signed
sub
 */

/* example
fn add(b: usize,
        p: usize,
        q: usize,
        c: bool,
        hb: usize,
        cb: usize) -> (usize, bool, bool, bool) {
    let c = c as usize;
    let m = (1 << b) - 1;
    let s = (p + q + c) & m;
    let h = carry(hb, p, q, c);
    let c = carry(cb, p, q, c);
    let z = s == 0;
    (s, h, c, z)
}
*/
