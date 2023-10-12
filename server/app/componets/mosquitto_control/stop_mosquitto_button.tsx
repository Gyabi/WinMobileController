"use client";
import {invoke} from "@tauri-apps/api/tauri";
import {message} from "@tauri-apps/api/dialog";

const StopMosquittoButton = () => {
    return (
        <div>
            <button className="bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded"
                onClick={stopMosquitto}>
                Stop Mosquitto
            </button>
        </div>
    );
}

const stopMosquitto = async () => {
    await invoke('stop_mosquitto').catch(async (err) => {
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

export default StopMosquittoButton;