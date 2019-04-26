namespace mazzaroth
{

  // Block conains a block header and transactions committed to this block
  struct Block
  {
    // Order is preserved in repeated fields
    BlockHeader header;
    Transaction transactions<>;
  }

  // BlockHeader contains fields that describe the block
  // TODO: Transaction Bloom and Receipt Bloom
  struct BlockHeader
  {

    string timestamp<256>; 

    unsigned hyper blockHeight;

    Hash txMerkleRoot;

    Hash txReceiptRoot;

    Hash stateRoot;

    Hash previousHeader;

    ID blockProducerAddress;
    
  }
}
