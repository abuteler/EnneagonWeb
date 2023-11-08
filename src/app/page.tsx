import Image from 'next/image'
import Link from 'next/link'

export default function Landing() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-evenly p-24">
      <Link className="group" href="/soon">
        <div className="relative flex flex-col place-items-center before:absolute before:h-[300px] before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#577ce2] after:dark:opacity-40 before:lg:h-[360px] z-[-1]">
          <Image
            src="/logo_landing.svg"
            alt="Enneagon Studios Logo"
            width={180}
            height={180}
            priority
          />
          <div className="relative flex gap-8 place-items-end ml-12 pt-8">
            <Image src="/mui_laptop.svg" alt="Laptop icon" width={45} height={45} priority />
            <Image src="/mui_tablet.svg" alt="Tablet icon" width={34} height={34} priority />
            <Image src="/mui_smartphone.svg" alt="Smartphone icon" width={16} height={16} className="pb-1" priority />
            <div className="w-5 h-6 invisible"></div>
            <Image
              src="/mui_present_to_all.svg"
              alt="Present to all icon"
              width={24}
              height={24}
              className="absolute right-7 mb-10 transition-[margin-bottom] ease duration-700 group-hover:mb-52"
              priority
            />
            <span className="text-xl animate-blink">_</span>
          </div>
        </div>
      </Link>
    </main>
  )
}
