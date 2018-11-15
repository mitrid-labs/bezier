use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Storable;

use crypto::Hasher;
use model::{UTxO, Transaction, Block, BlockNode, BlockChain};
use io::Store;
use io::network::{Message, Request, Response};
use io::network::message::request::transaction::*;
use io::network::message::request::block::*;
use io::network::server::handler::error::*;

pub fn create(store: &mut Store,
              request: &Request)
    -> Result<Response>
{
    request.check()?;

    if TransactionRequest::verify_create(request)? {
        let transaction = TransactionRequest::parse_create(request)?;
        transaction.check()?;

        if Transaction::store_lookup(store, &transaction.id)? {
            return error(store, request, "already found");
        }

        let mut new_utxos: Vec<UTxO> = Vec::new();

        for ref input in transaction.inputs.iter() {
            let coin = &input.coin;
            if !UTxO::store_lookup(store, &coin.id)? {
                if Transaction::store_lookup(store, &coin.tx_id)? {
                    return error(store, request, &format!("unknown transaction: {:?}", coin.tx_id));
                }
                new_utxos.push(coin.to_owned());
            }
        }

        for utxo in new_utxos {
            utxo.store_create(store)?;
        }

        transaction.store_create(store)?;
    } else if BlockRequest::verify_create(request)? {
        let block = BlockRequest::parse_create(request)?;
        block.check()?;

        if Block::store_lookup(store, &block.id)? {
            return error(store, request, "already found");
        }

        for tx in block.transactions.iter() {
            if !Transaction::store_lookup(store, &tx.id)? {
                return error(store, request, &format!("transaction not found: {:?}", tx.id));
            }
        }

        let blockchains = BlockChain::store_list(store, None, None, None, 0)?;
        if blockchains.len() > 1 {
            return error(store, request, "internal server error");
        }

        let mut blockchain = blockchains[0].to_owned();

        if blockchain.tip_idx != Some(0) {
            return error(store, request, "internal server error");
        }

        if blockchain.frontier_len != 1 {
            return error(store, request, "internal server error");
        }

        let mut hasher = Hasher{};

        if blockchain.height < block.height {
            blockchain.height = block.height;
            let blocknode = BlockNode::new()
                                .meta(&block.meta)?
                                .block_data(&block.id, block.height)?
                                .finalize(&mut hasher)?;

            blockchain.frontier = vec![blocknode];
            blockchain = blockchain.finalize(&mut hasher)?;
            blockchain.store_create(store)?;
        } 

        block.store_create(store)?;
    } else {
        return error(store, request, "invalid resource");
    };

    let mut hasher = Hasher{};

    let message = Message::new()
                        .meta(&request.message.meta)?
                        .session(&request.message.session)?
                        .method(&request.message.method)?
                        .resource(&request.message.resource)?
                        .payload(&().to_bytes()?)?
                        .finalize(&mut hasher)?;

    Response::new(&message)
}