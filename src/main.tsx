import  {StrictMode} from "react"
import ReactDOM from "react-dom/client"
import {RouterProvider ,createRouter} from "@tanstack/react-router";
import "./main.css"
import {rspc, client, queryClient} from "@/lib/rspc.ts";


import {routeTree} from "./routeTree.gen"

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
                <RouterProvider router={router} />
            </rspc.Provider>

        </StrictMode>
    )
}

