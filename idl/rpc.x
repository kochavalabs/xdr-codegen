%#include "common.x"
%#include "block.x"
%#include "receipt.x"
%#include "transaction.x"

namespace mazzaroth
{
  typedef string StatusInfo<256>;

  enum IdentifierType
  {
    NUMBER = 0,
    HASH = 1
  };

  union Identifier switch (IdentifierType type)
  {
    case NUMBER:
      unsigned hyper number;
    case HASH:
      Hash hash;
  };

  struct BlockLookupRequest
  {
    Identifier id; 
  };

  struct BlockHeaderLookupRequest
  {
    Identifier id; 
  };

  struct BlockLookupResponse
  {

    // Block that was requested if status is found.
    Block block;

    // Status for the requested block.
    BlockStatus status;

    // Human readable information to help understand the block status.
    StatusInfo statusInfo;
  };

  struct BlockHeaderLookupResponse
  {

    // Block header that was requested if status is found.
    BlockHeader header;

    // Status for the requested block.
    BlockStatus status;

    // Human readable information to help understand the block status.
    StatusInfo statusInfo;

  };

  // Status of a block.
  enum BlockStatus
  {

    // Status of the block is unkown.
    UNKNOWN = 0,

    // This block has been created.
    CREATED = 1,

    // This block has not been created yet.
    FUTURE = 2,

    // The block that was looked up was not found.
    NOT_FOUND = 3
  };

  // Request for a node to look up the status and value of a transaction.
  struct TransactionLookupRequest 
  {
    // Unique transaction identifier.
    ID transactionId;
  };

  // Response to lookup request.
  struct TransactionLookupResponse
  {
    // Final transaction written to the blockchain.
    Transaction transaction;

    // Current status of the transaction.
    TransactionStatus status;

    // Human readable information to help understand the transaction status.
    StatusInfo statusInfo;
  };

  // Message sent to a node to submit a transaction.
  struct TransactionSubmitRequest
  {
    Transaction transaction;
  };

  // Response from a node from a transaction Request.
  struct TransactionSubmitResponse
  {
    // Final transaction written to the blockchain. (if successful)
    ID transactionId;

    // Current status of the transaction.
    TransactionStatus status;

    // Human readable information to help understand the transaction status.
    StatusInfo statusInfo;
  };

  // Status of a transaction.
  enum TransactionStatus
  {

    // The transaction status is either not known or not set.
    UNKNOWN = 0,

    // The transaction has been accepted by a node and is in the process of being
    // submitted to the blockchain.
    ACCEPTED = 1,

    // This transaction was not accepted by the blockchain.
    REJECTED = 2,

    // The transaction has succesfully been added to the blockchain.
    CONFIRMED = 3,

    // This transaction was not found.
    NOT_FOUND = 4
  };

  // Request for a node to look up a transaction receipt.
  struct ReceiptLookupRequest 
  {
    // Unique transaction identifier.
    ID transactionId;
  };

  // Response to receipt lookup request.
  struct ReceiptLookupResponse
  {
    // Final receipt written to the blockchain.
    Receipt receipt; 

    // Current status of the receipt
    ReceiptLookupStatus status;

    // Human readable information to help understand the receipt status.
    StatusInfo statusInfo;
  };

  // Status of a receipt.
  enum ReceiptLookupStatus
  {
    // The receipt status is either not known or not set.
    UNKNOWN = 0,

    // The transaction receipt was found
    FOUND = 1,

    // This transaction receipt was not found.
    NOT_FOUND = 2
  };

}
