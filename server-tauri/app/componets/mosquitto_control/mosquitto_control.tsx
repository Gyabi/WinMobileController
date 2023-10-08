import StartMosquittoButton from "./start_mosquitto_button";
import StopMosquittoButton from "./stop_mosquitto_button";

const MosquittoControl = () => {
    return (
        <div className="flex flex-row">
            <div className="p-10">
                <StartMosquittoButton />
            </div>

            <div className="p-10">
                <StopMosquittoButton />
            </div>
        </div>
    );
}

export default MosquittoControl;