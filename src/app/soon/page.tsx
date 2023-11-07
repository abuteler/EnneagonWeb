import Image from 'next/image'

export default function ComingSoon() {
  return(
    <main className="flex min-h-screen flex-col items-center justify-evenly p-24">
      <div className="relative flex flex-col place-items-center before:absolute before:h-[300px] before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#577ce2] after:dark:opacity-40 before:lg:h-[360px] z-[-1]">
        <Image
          src="/logo_landing.svg"
          alt="Enneagon Studios Logo"
          // className="light:invert"
          width={240}
          height={240}
          priority
        />
        <Image
          src="/company_title_2023.svg"
          alt="company title"
          width={433}
          height={90}
          priority
        />
        <div className="relative text-xl flex gap-2 place-items-center pt-6">
          Coming soon
          <div className="inline animate-blink">_</div>
        </div>
      </div>
    </main>
  )
}