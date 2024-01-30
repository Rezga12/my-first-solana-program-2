import {
    clusterApiUrl,
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction
} from "@solana/web3.js";
import {readOnlyKey, signerKey, writeableKey} from "./utils/instruction-utils";
import {
    createAssociatedTokenAccount,
    createAssociatedTokenAccountInstruction,
    TOKEN_PROGRAM_ID
} from "@solana/spl-token";

const connection = new Connection(clusterApiUrl('devnet'));
const keypair = Keypair.fromSecretKey(Uint8Array.from([]))

const programId = new PublicKey('JJJ1zWC9EzfoDsJQPQdgkLGRseBgcDd62nNriE4m46w');

// calculate pda
const [pda, bump] = PublicKey.findProgramAddressSync([Buffer.from("payer_seeds")], programId);

// console.log('pda: ', pda);

async function main(){
    const pdaTokenAccount = new PublicKey('AbDkZ2p9dEMHxufv4N2JT9fqtwpUoPMWpRdaeWurQpWV');
    const ix = new TransactionInstruction({
        programId,
        keys: [
            {
                pubkey: pda,
                isWritable: true,
                isSigner: false,
            },
            {
                pubkey: new PublicKey('29zMU3FKQt7cSgfUK1rneDUc82R1hVPHoYfkhWa9L1uc'),
                isWritable: true,
                isSigner: false
            },
            {
                pubkey: SystemProgram.programId,
                isWritable: false,
                isSigner: false
            },
            {
                pubkey: TOKEN_PROGRAM_ID,
                isWritable: false,
                isSigner: false
            },
            {
                pubkey: pdaTokenAccount,
                isWritable: true,
                isSigner: false,
            },
            {
                pubkey: new PublicKey('3q5WqCdjRG3YVk6LTxfX2DMhkph64qMCvcNfMwwbj3Ex'),
                isSigner: false,
                isWritable: true
            }
        ],
        data: Buffer.from([])
    });

    const tx = new Transaction();
    tx.add(ix);

    tx.feePayer = keypair.publicKey;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

    tx.sign(keypair);

    const txId = await connection.sendRawTransaction(tx.serialize(), {skipPreflight: false});

    console.log('txId: ', txId);

}

main();
