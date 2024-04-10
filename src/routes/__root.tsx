import { createRootRoute, Link, Outlet} from "@tanstack/react-router"
import {TanStackRouterDevtools} from "@tanstack/router-devtools";


export const Route = createRootRoute({
    component: () => {
        return (
            <>
                <div className="text-red-500">hello</div>Hello
                <Outlet />
            </>
        )
    }
})