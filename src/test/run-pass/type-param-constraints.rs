fn p_foo<T>(pinned: T) { }
fn s_foo<copy T>(shared: T) { }
fn u_foo<send T>(unique: T) { }

resource r(i: int) { }

fn main() {
    p_foo(r(10));
    p_foo(@r(10));

    p_foo(~r(10));
    p_foo(@10);
    p_foo(~10);
    p_foo(10);

    s_foo(@r(10));
    s_foo(@10);
    s_foo(~10);
    s_foo(10);

    u_foo(~10);
    u_foo(10);
}
