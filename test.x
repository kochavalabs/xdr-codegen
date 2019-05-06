
%#include "common.x"

namespace mazzaroth
{

  typedef opaque Signature[64];
  typedef opaque ID[32];
  typedef opaque Hash[32];
  typedef opaque Parameter<>;

  // A transaction that calls a function on a user defined contract.
  struct Call
  {
    // Contract function to execute.
    string function<256>;

    // Parameters to the contract function. The serialization format is defined
    // by the contract itself.
    Parameter parameters<>;
  };

  // A transaction that provides a contract as a wasm binary.
  struct Update
  {
    // Contract binary bytes.
    opaque contract<>;
  };

  enum ActionCategoryType
  {
    NONE = 0,
    CALL = 1,
    UPDATE = 2
  };

  union ActionCategory switch (ActionCategoryType Type)
  {
    case NONE:
      void;
    case CALL:
      Call call;
    case UPDATE:
      Update update;
  };

  // The Action data of a transaction
  // Set by the client to form a transaction
  struct Action 
  {
    ID channelId;    

    unsigned hyper nonce;

    ActionCategory category;

  };

  // A transaction that calls a function on a user defined contract.
  struct Transaction
  {
    // Byte array signature of the Transaction bytes signed by the Transaction 
    // sender's private key.
    Signature signature;

    // Byte array representing the id of the sender, this also happens
    // to be the sender's account public key.
    ID address[12];

    // The action data for this transaction
    Action action<3>;
  };


  // A transaction that has been executed, contains a receipt, and is
  // ready to be stored in the ledger.
  struct CommittedTransaction
  {
    // The transaction itself
    Transaction transaction;

    // The execution ordering of the transaction, provided by consensus
    unsigned hyper sequenceNumber[12];

    // The receipt hash generated by consensus, to be compared to local execution results
    ID receiptId[12];

    // The transaction merkle root after adding this transaction to the merkle tree, for validation
    Hash currentTransactionRoot<23>;

     // Consensus signatures
    Signature signatures<>;
  };
}
