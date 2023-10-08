"use client";
import {invoke} from "@tauri-apps/api/tauri";
import {message} from "@tauri-apps/api/dialog";
import { useState } from "react";

const ServerButton = () => {
    const [isRunning, setIsRunning] = useState(false);

    const startServer = async () => {
        await invoke('start_server')
        .then(async (res) => {
            setIsRunning(true);
        })
        .catch(async (err) => {
            await message(
                err,
                {
                    title: 'Server Start Error',
                    type: 'error'
                }
            );
        }); 
    }
    
    const stopServer = async () => {
        await invoke('stop_server')
        .then(async (res) => {
            setIsRunning(false);
        })
        .catch(async (err) => {
            await message(
                err,
                {
                    title: 'Server Stop Error',
                    type: 'error'
                }
            );
            return;
        });
    }

    return (
        <div>
            {isRunning ? (
                <button className="w-72 h-72 relative"
                    onClick={stopServer}>
                    <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
                        <div className="w-40 h-40 rounded-full bg-purple-500 animate-ping"></div>
                    </div>
                    <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
                        <div className="w-72 h-72 rounded-full bg-purple-500 hover:bg-purple-600"></div>
                    </div>
                    <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
                        <p className="font-bold text-white">
                            RUNNING
                        </p>
                    </div>
                </button>
            ):(
                <button className="w-72 h-72 rounded-full bg-blue-500 hover:bg-blue-600 text-white"
                    onClick={startServer}>
                    <p className="font-bold">
                        START
                    </p>
                </button>
            )}
        </div>
    )
}


export default ServerButton;