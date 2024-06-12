// when a process starts it requires access to a small part of memory in RAM (rust pushes variables there)
// some variables size is definite some not
fn main() {
  let z = String::from("asd");
  z.push_str("skjdkjfbjkbgjkfgk")// we see size changing so we dont save it on stack thus stored on heap(we have a pointer to heap data on the stack))
}