mod chore;

use chore::mutex::test_mutex;
use chore::rwlock::test_rwlock;
use chore::refcell::test_refcell;

fn main() {

    test_mutex();
    test_rwlock();
    test_refcell();
}