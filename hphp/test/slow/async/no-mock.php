<?hh

<<__MockClass>>
class MyWaitHandle extends WaitableWaitHandle {
  public function __construct() {
    echo "Ha ha!\n";
  }
}

try {
  $wh = new MyWaitHandle;
} catch (Exception $e) {
  echo "Exception: ", $e->getMessage(), "\n";
}
