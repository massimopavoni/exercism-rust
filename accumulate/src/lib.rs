pub fn map<Map, A, B>(input: Vec<A>, function: Map) -> Vec<B>
where
    Map: FnMut(A) -> B,
{
    input.into_iter().map(function).collect()
}
