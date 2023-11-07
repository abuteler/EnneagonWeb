import Image from 'next/image'
import Link from 'next/link'
import { Laptop, Tablet, Smartphone, PresentToAll } from '@mui/icons-material'

export default function Landing() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-evenly p-24">
      <div className="relative z-10 flex flex-col place-items-center before:absolute before:h-[300px] before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#577ce2] after:dark:opacity-40 before:lg:h-[360px] z-[-1]">
        <Image
          src="/logo_landing.svg"
          alt="Enneagon Studios Logo"
          width={180}
          height={180}
          priority
        />
        <div className="relative z-20 flex gap-7 place-items-end ml-14 pt-6">
          <Laptop className="text-5xl" />
          <Tablet className="text-4xl mb-1" />
          <Smartphone className="text-3xl mb-2" />
          <Link className="link" href="/soon">
            <div className="group absolute bottom-0 right-0 h-64 w-64 flex place-items-end justify-end">
              <PresentToAll className="pb-10 mr-5 h-auto transition-[padding-bottom] ease duration-700 group-hover:pb-52" />
            </div>
          </Link>
          <div className="text-2xl animate-blink">_</div>
        </div>
      </div>
    </main>
  )
}
