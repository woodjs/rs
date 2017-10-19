mod rc;
mod arc;
mod mutex;
mod rwlock;
mod cell;
mod refcell;

use mutex::test_mutex;
use rwlock::test_rwlock;
use refcell::test_refcell;

fn main() {

    test_mutex();
    test_rwlock();
    test_refcell();
}