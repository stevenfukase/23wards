import React, { useState } from 'react';
import { useWard } from './GlobalContext';
import Navbar from './components/Navbar';

export default function App() {
  const { state } = useWard();
  const [loading, setLoading] = useState(false);
  const [currentWardObj, setCurrentWardObj] = useState();

  const randomIntFromInterval = (min, max) => Math.floor(Math.random() * (max - min + 1) + min);

  const generateHandler = () => {
    setLoading(true);
    const randInt = randomIntFromInterval(0, state.length - 1);
    setCurrentWardObj(state[randInt]);
    setLoading(false);
  };

  return (
    <div className="h-screen dark:bg-gray-900">
      <Navbar />
      <div className="pt-16 h-full grid sm:grid-cols-2 divide-x divide-gray-200 dark:divide-gray-400">
        <div className="place-self-center transform sm:-translate-y-12 flex flex-col items-center">
          {!currentWardObj && <h1 className="dark:text-white">Welcome to Random Tokyo Ward generator</h1>}

          {currentWardObj
            && (
              <>
                <h1 className="text-6xl dark:text-white">{currentWardObj.ward}</h1>
                <h2 className="text-4xl dark:text-white">{currentWardObj.japanese}</h2>
              </>
            )}

          {!loading
            && (
            <button
              type="button"
              className="focus:outline-none text-gray-500 dark:text-gray-400"
              onClick={generateHandler}
            >
              Generate Place
            </button>
            )}

        </div>
        <div>
          {loading && <h1>Loading...</h1>}
          {currentWardObj
          && (
          <iframe
            title="map"
            className={`h-full w-full ${loading && 'hidden'}`}
            src={`https://www.google.com/maps/embed/v1/place?key=AIzaSyDjd3XyCKvPTWNeIKtEWJpUCDW874-XBvM&q=${encodeURIComponent(currentWardObj.ward)}`}
            allowFullScreen
          />
          )}
        </div>
      </div>
    </div>
  );
}
