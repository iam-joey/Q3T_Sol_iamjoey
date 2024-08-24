import {
  ActionGetRequest,
  ActionGetResponse,
  ActionPostRequest,
  ActionPostResponse,
  ACTIONS_CORS_HEADERS,
  createPostResponse,
  MEMO_PROGRAM_ID,
} from "@solana/actions";
import {
  clusterApiUrl,
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

export const GET = () => {
  const payload: ActionGetResponse = {
    icon: "https://solana-actions.vercel.app/solana_devs.jpg",
    description: "hello world",
    label: "asdasdasd",
    title: "asdasd",
    links: {
      actions: [
        {
          href: "/api/actions/message?message={message}",
          label: "Send message",
          parameters: [
            {
              name: "message",
              label: "enter the message you wanna send",
              required: true,
            },
          ],
        },
      ],
    },
    type: "action",
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
    console.log("here");
    try {
      account = new PublicKey(body.account);
    } catch (err) {
      throw "Invalid 'account' provided. Its not a real pubkey";
    }
    var msg = "";
    if (url.searchParams.has("message")) {
      msg = url.searchParams.get("message") || "hello bro";
    }

    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    const transaction = new Transaction();

    transaction.add(
      new TransactionInstruction({
        keys: [{ pubkey: account, isSigner: true, isWritable: true }],
        programId: new PublicKey(MEMO_PROGRAM_ID),
        data: Buffer.from(`${msg}`, "utf-8"),
      })
    );

    transaction.feePayer = account;
    transaction.recentBlockhash = (
      await connection.getLatestBlockhash()
    ).blockhash;

    const payload: ActionPostResponse = await createPostResponse({
      fields: {
        transaction,
        message: "all wewnt good in message1",
        links: {
          next: {
            type: "inline",
            action: {
              icon: "https://avatars.githubusercontent.com/u/38688596?s=400&u=537ec3624a74119be8caba48e5ee38610ad1717a&v=4",
              description: "hello world 2",
              label: "asdasdasd",
              title: "asdasd",
              links: {
                actions: [
                  {
                    label: "send message to someone esle",
                    href: "/api/actions/message2?message=hello from messag2",
                  },
                  {
                    href: "/api/actions/message2?message={message}",
                    label: "Send message",
                    parameters: [
                      {
                        name: "message",
                        label: "enter the message you wanna send",
                        required: true,
                      },
                    ],
                  },
                ],
              },
              type: "action",
            },
          },
        },
      },
    });

    return Response.json(payload, {
      headers: ACTIONS_CORS_HEADERS,
    });
  } catch (error) {
    console.log(error);
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
