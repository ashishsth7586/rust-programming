import argparse
import pandas as pd

from solana.rpc.api import Client
from spl.token.constants import TOKEN_PROGRAM_ID
from spl.token.instructions import transfer_checked, get_associated_token_address, create_associated_token_account, TransferCheckedParams # Add accordingly: transfer

from solana.rpc.commitment import Confirmed
from solana.publickey import PublicKey
from solana.rpc.types import TxOpts
from solana.keypair import Keypair
from solana.transaction import Transaction

ZEBEC_TOKEN_MINT_ADDRESS = "GFaCSR2zttiQuuv76Ba48R8BCv6i8eakGofkctcqEC9J" # example token mint address
RPC_CLUSTER_URL = "https://api.devnet.solana.com"

fromWallet = Keypair.from_secret_key(b'\xc0\xc1H\xd2~u\n\xd7l!\xd5~\x8b\\\xecQ\xbdm\x84\xfd\xb01|\x9d\xe4\xa9\x99q\x7fy\xfb\xeb')

def get_or_create_associated_account(client, wallet_address, mintAddress):

    """
    returns associated token account, for a given wallet address & Token Mint Address.
    ::args::
    client: Solana Client Object
    wallet_address: string -> a valid base-64 encoded wallet_address
    mintAddress: string -> SPL Token Mint Address.
    """
    walletAddressPubKey = PublicKey(wallet_address)
    tokenMintAddress = PublicKey(mintAddress)

    tokenAddress = get_associated_token_address(walletAddressPubKey, tokenMintAddress)

    tokenAddressAccountInfo = client.get_account_info(
        pubkey=PublicKey(tokenAddress), 
        encoding="base64"
    )

    accountAddress = None

    if tokenAddressAccountInfo['result']['value'] is None:
        print("no associated account data found")
        print(f"creating an associated account for {mintAddress}")

        ix = create_associated_token_account(
            payer=fromWallet.public_key,
            owner=walletAddressPubKey,
            mint=tokenMintAddress
        )

        tx = Transaction().add(ix)
        recent_blockhash = client.get_recent_blockhash()

        if recent_blockhash['result']['value'] is None:
            raise Exception("could not fetch recent blockhash")

        tx.recent_blockhash = recent_blockhash['result']['value']['blockhash']
        response = client.send_transaction(tx, fromWallet)

        if response['result'] is not None:
            account_address = get_associated_token_address(walletAddressPubKey)
        else:
            raise Exception("failed to send transaction. Internal Server Error!")
    else:
        account_address = get_associated_token_address(walletAddressPubKey, tokenMintAddress)
    
    return account_address


def transfer_token(client, mintAddress, toWallet, amount, decimals=9):

    """
    transfer SPL token to a wallet address
    ::args::
    client: Solana Client Object
    mintAddress: string -> Token Mint Address
    toWallet: string -> Receipient Wallet Address
    amount: Actual Amount (Note: not in Lamports)
    decimals: Default is 9, could be 6 as well.
    """

    toWalletPubKey = PublicKey(toWallet)
    mintAddressPubKey = PublicKey(mintAddress)
    fromTokenAddress = get_or_create_associated_account(client, fromWallet.public_key, mintAddressPubKey)
    toTokenAddress = get_or_create_associated_account(client, toWalletPubKey, mintAddressPubKey)

    totalAmount = amount * pow(10, decimals)

    transfer_params = TransferCheckedParams(
        program_id = TOKEN_PROGRAM_ID,
        source = fromTokenAddress,
        mint = mintAddressPubKey,
        dest = toTokenAddress,
        owner = fromWallet.public_key,
        amount = totalAmount,
        decimals = decimals,
        signers = [fromWallet.public_key]
    )

    ix = transfer_checked(transfer_params)
    tx = Transaction().add(ix)

    signature = None

    try:
        signature = client.send_transaction(
            tx, 
            fromWallet, 
            opts=TxOpts(skip_confirmation=False, preflight_commitment=Confirmed)
        )
    except Exception as e:
        return {
            "status": "error",
            "message": e,
            "result": {
                "transactionHash": None
            }
        }

    return {
        "status": "success",
        "message": f"successfully transfered {amount} to {toWallet}",
        "result": {
            "transactionHash": signature['result']
        }
    }


if __name__ == "__main__":
	
    parser = argparse.ArgumentParser(
        description="CSV file location and Token Mint Address."
    )

    parser.add_argument(
        "-f", "--file",
        metavar="",
        dest="csv_file_location",
        type=str,
        required=True,
        help="relative file location of CSV file."
    )

    parser.add_argument(
        "-t", "--token-mint-address",
        metavar="",
        dest="token_mint_address",
        required=False,
        type=str,
        help="SPL token mint address"
    )

    args = parser.parse_args()

    if args.csv_file_location == None:
        print("file location of CSV is required!")
        sys.exit(1)

    if args.token_mint_address == None:
        print(f"using default token: {ZEBEC_TOKEN_MINT_ADDRESS}")
        args.token_mint_address = ZEBEC_TOKEN_MINT_ADDRESS

    print(args)

    sol_client = Client(endpoint=RPC_CLUSTER_URL, commitment=Confirmed)

    df = pd.read_csv(args.csv_file_location)

    for index, row in df.iterrows():
        toWallet = row.recipient
        amount = row.amount
        response = transfer_token(sol_client, ZEBEC_TOKEN_MINT_ADDRESS, toWallet, amount)
        df.loc[index, 'transactionHash'] = response['result']['transactionHash']

    df.to_csv("wallet_response.csv", index=False)