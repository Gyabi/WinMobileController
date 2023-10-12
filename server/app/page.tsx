import MosquittoControl from "./componets/mosquitto_control/mosquitto_control"
import ServerControl from "./componets/server_control/server_control"
import SettingButton from "./componets/setting/setting_button"

export default function Home() {
  return (
    <main>
      <nav className="bg-white border-gray-200 dark:bg-gray-900">
        <div className="flex flex-wrap justify-between items-center mx-auto max-w-screen-xl p-4">
          <p className="antialiased font-bold tracking-wide font-sans text-2xl underline underline-offset-8">WinMobileController</p>
            <div className="flex items-center">
              <SettingButton />
            </div>
        </div>
      </nav>
      <div className="flex min-h-screen flex-col items-center p-10">
        <div className="my-5">
          <MosquittoControl />
        </div>
        <div className="my-5">
          <ServerControl />
        </div>
      </div>
    </main>
  )
}
