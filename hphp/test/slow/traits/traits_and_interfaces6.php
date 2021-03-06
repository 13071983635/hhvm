<?hh

interface I {
  public function foo();
  public function bar();
}

trait T implements I {
  public function foo() {
    echo "T::foo\n";
  }
  public function bar() {
    echo "T::bar\n";
  }
}

class C implements I {
  use T;

  public function bar() {
    echo "C::bar\n";
  }
}

function xyz(I $x) {
  $x->foo();
  $x->bar();
}

function main() {
  $c = new C();
  xyz($c);
}


<<__EntryPoint>>
function main_traits_and_interfaces6() {
error_reporting(-1);

main();
}
