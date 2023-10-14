"use client";
import { invoke } from "@tauri-apps/api/tauri";
import {message} from "@tauri-apps/api/dialog";

const StartMosquittoButton = () =>{
    return(
        <div>
            <button className="bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded"
                onClick={startMosquitto}>
                Start Mosquitto
            </button>
        </div>
    )
}

const startMosquitto = async () => {
    await invoke('start_mosquitto').catch(async (err) => {
        await message(
            err,
            {
                title: 'Info',
                type: 'info'
            }
        );
        return;
    });
}

export default StartMosquittoButton;