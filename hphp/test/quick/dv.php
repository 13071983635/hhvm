<?hh

error_reporting(0);

function foo($a, $b=22, $c=33) {
  echo "$a/$b/$c\n";
}

function main() {
  foo(1, 2, 3, 4);
  foo(1, 2, 3);
  foo(1, 2);
  foo(1);
  try { foo(); } catch (Exception $e) { var_dump($e->getMessage()); }

  function out_of_order($a=44, $b, $c=66) {
    echo "$a/$b/$c\n";
  }

  out_of_order(4, 5, 6, 7);
  out_of_order(4, 5, 6);
  out_of_order(4, 5);
  try { out_of_order(4); } catch (Exception $e) { var_dump($e->getMessage()); }
  try { out_of_order(); } catch (Exception $e) { var_dump($e->getMessage()); }
}
main();
