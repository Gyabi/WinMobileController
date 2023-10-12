import ServerButton from "./server_button";
// ボタンとステータスを表示するコンポーネント
const ServerControl = () => {
    return (
        <div className="flex flex-col items-center">
            <div className="flex flex-row">
                <ServerButton />
            </div>
        </div>
    )
}

export default ServerControl;