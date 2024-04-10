import { createClient } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react";
import { QueryClient } from "@tanstack/react-query";
import { Procedures } from "./procedures";
import { TauriTransport } from "@rspc/tauri";


export const queryClient = new QueryClient();

export const client = createClient<Procedures>({
  transport: new TauriTransport()
});

export const rspc = createReactQueryHooks<Procedures>();
