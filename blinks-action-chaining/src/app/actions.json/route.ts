import { ACTIONS_CORS_HEADERS, ActionsJson } from "@solana/actions";

export const GET = () => {
  const payload: ActionsJson = {
    rules: [
      {
        pathPattern: "/message",
        apiPath: "/api/actions/message",
      },
      {
        pathPattern: "/message2",
        apiPath: "/api/actions/message2",
      },
    ],
  };

  return Response.json(payload, {
    headers: ACTIONS_CORS_HEADERS,
  });
};

export const OPTIONS = GET;
