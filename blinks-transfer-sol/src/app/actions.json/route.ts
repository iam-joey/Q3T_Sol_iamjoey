import { ACTIONS_CORS_HEADERS, ActionsJson } from "@solana/actions";

export const GET = async (req: Request) => {
  const payload: ActionsJson = {
    rules: [
      {
        pathPattern: "/coffee",
        apiPath: "/api/actions/donate",
      },
    ],
  };

  return Response.json(payload, {
    headers: ACTIONS_CORS_HEADERS,
  });
};

export const OPTIONS = GET;
