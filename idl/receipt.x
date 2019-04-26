%#include "common.x"
%#include "event.x"

namespace mazzaroth
{
  struct Receipt
  {
    // Status failure or success
    ReceiptStatus status;
 
    // The state root after execution of the transaction
    Hash stateRoot;
 
    // The list of events fired during execution of this transaction
    Event events<>;
 
    // Return results of execution if there is one for function called
    opaque result<>;
  };

  enum ReceiptStatus
  {
    FAILURE = 0,
    SUCCESS = 1
  };
}
