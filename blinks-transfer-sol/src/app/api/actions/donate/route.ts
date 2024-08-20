import {
  ActionGetResponse,
  ActionPostRequest,
  ActionPostResponse,
  ACTIONS_CORS_HEADERS,
  createPostResponse,
} from "@solana/actions";
import {
  clusterApiUrl,
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";

export const GET = async () => {
  const payload: ActionGetResponse = {
    icon: "https://solana-actions.vercel.app/solana_devs.jpg",
    title: "Transfer SOL",
    label: "Send me something",
    description: "Just send me some SOL for living",
    links: {
      actions: [
        {
          href: "/api/actions/donate?amount=0.1",
          label: "0.1 SOL",
        },
        {
          href: "/api/actions/donate?amount=0.5",
          label: "0.5 SOL",
        },
        {
          href: "/api/actions/donate?amount={amount}",
          label: "SEND SOL",
          parameters: [
            {
              name: "amount",
              label: "Enter the SOl amount",
              required: true,
            },
          ],
        },
      ],
    },
  };

  return Response.json(payload, {
    headers: ACTIONS_CORS_HEADERS,
  });
};

export const OPTIONS = GET;

export const POST = async (req: Request) => {
  try {
    const url = new URL(req.url);

    const body: ActionPostRequest = await req.json();

    let account: PublicKey;
    try {
      account = new PublicKey(body.account);
    } catch (err) {
      throw "Invalid 'account' provided. Its not a real pubkey";
    }
    let amount: number = 0.1;
    if (url.searchParams.has("amount")) {
      try {
        amount = parseFloat(url.searchParams.get("amount") || "0.1");
        console.log(amount);
      } catch (error) {
        throw "invalid amount";
      }
    }

    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    const transaction = new Transaction();

    transaction.add(
      SystemProgram.transfer({
        fromPubkey: account,
        lamports: amount * LAMPORTS_PER_SOL,
        toPubkey: new PublicKey("DLgacSweX6fmAbnzwoFnVwcuGRMwFvdCzzwhrXuE5pPc"),
      })
    );

    transaction.feePayer = account;
    transaction.recentBlockhash = (
      await connection.getLatestBlockhash()
    ).blockhash;

    const payload: ActionPostResponse = await createPostResponse({
      fields: {
        transaction,
        message: "Thanks for the donation",
      },
    });

    return Response.json(payload, {
      headers: ACTIONS_CORS_HEADERS,
    });
  } catch (error) {
    console.log("error", error);
    return Response.json(
      {
        msg: "Something went wrong",
      },
      {
        headers: ACTIONS_CORS_HEADERS,
      }
    );
  }
};
