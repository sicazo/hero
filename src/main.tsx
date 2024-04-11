import  {StrictMode} from "react"
import ReactDOM from "react-dom/client"
import {RouterProvider ,createRouter} from "@tanstack/react-router";
import "./main.css"
import {rspc, client, queryClient} from "@/lib/rspc.ts";


import {routeTree} from "./routeTree.gen"
import {ThemeProvider} from "@/components/theme/theme_provider.tsx";

const router = createRouter({ routeTree })


declare module '@tanstack/react-router'  {
    interface Register {
        router: typeof router
    }
}


const rootElement = document.getElementById('root')!
if (!rootElement.innerHTML) {
    const root = ReactDOM.createRoot(rootElement);
    root.render(
        <StrictMode>
            <rspc.Provider client={client} queryClient={queryClient}>
                <ThemeProvider defaultTheme={"dark"}>
                    <RouterProvider router={router} />
                </ThemeProvider>

            </rspc.Provider>

        </StrictMode>
    )
}

