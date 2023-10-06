import React from "react";

interface Data {
  storyNotes: string;
  locationsNotes: string;
  characterBuildNotes: string;
}

enum Tab {
  StoryNotes = `storyNotes`,
  LocationsNotes = `locationsNotes`,
  CharacterBuildNotes = `characterBuildNotes`,
}

function App() {

  const [data, setData] = React.useState<Data>(JSON.parse(localStorage.getItem(`data`) ?? `null`) ?? {
    storyNotes: ``,
    locationsNotes: ``,
    characterBuildNotes: ``,
  });

  const [selectedTab, setSelectedTab] = React.useState<Tab>(Tab.StoryNotes);

  return (
    <div
      style={{
        height: `100vh`,
        display: `flex`,
        flexDirection: `column`,
        justifyContent: `end`,
      }}
    >
      <div
        style={{
          height: `50vh`,
          backgroundColor: `#242424c3`,
          borderTopLeftRadius: `8px`,
          borderTopRightRadius: `8px`,
          padding: `8px`,
          display: `flex`,
          flexDirection: `column`,
        }}
      >
        <div
          style={{
            display: `flex`,
            flexDirection: `row`,
            justifyContent: `space-evenly`,
          }}
        >
          <h3
            style={{
              color: selectedTab === Tab.StoryNotes ? `#fff` : `#aaa`,
              cursor: `pointer`,
              userSelect: `none`,
            }}
            onClick={() => setSelectedTab(Tab.StoryNotes)}
          >
            Story Notes
          </h3>
          <h3
            style={{
              color: selectedTab === Tab.LocationsNotes ? `#fff` : `#aaa`,
              cursor: `pointer`,
              userSelect: `none`,
            }}
            onClick={() => setSelectedTab(Tab.LocationsNotes)}
          >
            Locations Notes
          </h3>
          <h3
            style={{
              color: selectedTab === Tab.CharacterBuildNotes ? `#fff` : `#aaa`,
              cursor: `pointer`,
              userSelect: `none`,
            }}
            onClick={() => setSelectedTab(Tab.CharacterBuildNotes)}
          >
            Character Build Notes
          </h3>
        </div>
        <textarea
          style={{
            padding: `8px`,
            height: `100%`,
            resize: `none`,
            fontSize: `24px`,
            fontFamily: `Inter, system-ui, Avenir, Helvetica, Arial, sans-serif`,
          }}
          spellCheck={false}
          key={selectedTab}
          value={
            selectedTab === Tab.StoryNotes ? data.storyNotes :
              selectedTab === Tab.LocationsNotes ? data.locationsNotes :
                data.characterBuildNotes
          }
          onChange={(e) => {
            const newData = {
              ...data,
              [selectedTab]: e.target.value,
            };
            setData(newData);
            localStorage.setItem(`data`, JSON.stringify(newData));
          }}
        >
        </textarea>
      </div>
    </div>
  )
}

export default App
