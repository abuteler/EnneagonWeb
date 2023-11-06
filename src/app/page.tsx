import Image from 'next/image'
import Link from 'next/link'
import { Laptop, Tablet, Smartphone, PresentToAll } from '@mui/icons-material'

export default function Landing() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-evenly p-24">
      
      <Link className="link" href="/soon">
      <div className="relative flex flex-col place-items-center before:absolute before:h-[300px] before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#0141ff] after:dark:opacity-40 before:lg:h-[360px] z-[-1]">
        <Image
            src="/logo_landing.svg"
            alt="Enneagon Studios Logo"
            className="light:invert"
            width={180}
            height={180}
            priority
          />
        <div className="relative flex gap-7 place-items-end ml-10 pt-6">
          <Laptop className="text-5xl" />
          <Tablet className="text-4xl mb-1" />
          <Smartphone className="text-3xl mb-2" />
          <Link className="link pl-3" href="/home">
            <div className="absolute bottom-0 right-5 h-16 hover:transition-[height] ease-in-out duration-100 hover:h-40">
              <PresentToAll className="" />
            </div>
          </Link>
          <div className="text-2xl transition-opacity duration-100">_</div>
        </div>
      </div>
      </Link>
      
    </main>
  )
}
