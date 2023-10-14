"use client";

import { invoke } from "@tauri-apps/api/tauri";
import { message } from "@tauri-apps/api/dialog";
import { useRouter } from "next/navigation";
import { useState, useEffect } from "react";
import { Parameter } from "../componets/setting/setting_button";

const Setting = () => {
    const router = useRouter();
    const [host, setHost] = useState('');
    const [port, setPort] = useState(0);
    const [caPath, setCaPath] = useState('');
    const [clientCertPath, setClientCertPath] = useState('');

    // useEffectで初期入力をRust側から取得
    useEffect(() => {
        invoke('get_setting')
            .then((param) => {
                const parameter: Parameter = JSON.parse(JSON.stringify(param));
                setHost(parameter.host);
                setPort(parameter.port);
                setCaPath(parameter.caPath);
                setClientCertPath(parameter.clientCertPath);
            })
            .catch((err) => {
                // ダイアログを表示
                message(
                    err,
                    {
                        title: 'Info',
                        type: 'info'
                    }
                );
            });
    }, []);

    const backClick = () => {
        router.push('/');
    }
    
    const submitClick = () => {
        // 入力値をRust側に保存
        invoke('set_setting', {parameter:{
            host: host,
            port: port,
            caPath: caPath,
            clientCertPath: clientCertPath
        }})
            .catch((err) => {
                // ダイアログを表示
                message(
                    err,
                    {
                        title: 'Info',
                        type: 'info'
                    }
                );
            })
            .then(() => {
                // 画面遷移
                router.push('/');
            });
    }

    const numCheck = (num:string) => {
        const parsedNum = parseInt(num.trim());
        if (isNaN(parsedNum)) {
            setPort(0);
        }
        else {
            setPort(parsedNum);
        }
    }

    return (
        <main>
            <div className="flex min-h-screen flex-col items-center p-10">
                <div className="py-5">
                    <p className="antialiased font-bold tracking-wide font-sans text-2xl underline underline-offset-8">Setting</p>
                </div>

                {/* 対応するinputを配置 */}
                <div>
                    <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">HOST</label>
                    <input value={host} onChange={(e) => setHost(e.target.value)} id="host" className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"></input>
                </div>
                <div>
                    <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">PORT</label>
                    <input value={port} onChange={(e) => numCheck(e.target.value)} id="port" className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"></input>
                </div>
                <div>
                    <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">CA PATH</label>
                    <input value={caPath} onChange={(e) => setCaPath(e.target.value)} id="caPath" className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"></input>
                </div>
                <div>
                    <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">CLIENT CERT PATH</label>
                    <input value={clientCertPath} onChange={(e) => setClientCertPath(e.target.value)} id="clientCertPath" className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"></input>
                </div>

                <div className="flex flex-row justify-between items-center">
                    <div className="p-10">
                        <button className="bg-gray-500 hover:bg-gray-400 text-white font-bold py-2 px-4 border-b-4 border-gray-700 hover:border-gray-500 rounded" onClick={backClick}>
                            Back
                        </button>
                    </div>
                    <div className="p-10">
                        <button className="bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded" onClick={submitClick}>
                            Save
                        </button>
                    </div>
                </div>
            </div>
        </main>
    );
}

export default Setting;