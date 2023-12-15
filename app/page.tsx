'use client'
import {useHeroStore} from "@/lib/HeroStore";
import {commands} from "@/lib/bindings";
import {useEffect} from "react";
import {invoke} from "@tauri-apps/api";

export default function Home() {

  const {test, updateTest} = useHeroStore()
   

  return (
      <>
          <p>{test}</p>
          <button onClick={() => updateTest(test + 1)}>Test</button>

      </>
  )
}
