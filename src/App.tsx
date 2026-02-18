import { Route, Routes, useLocation, useNavigate } from "react-router";
import PerformPage from "./pages/Perform";
import { Tabs, TabsList, TabsTrigger } from "./components/ui/tabs";
import ActorsPage from "./pages/Actors";

type Tab = {
  title: string;
  id: string;
  href: string;
};

const tabs: Tab[] = [
  {
    title: "Perform",
    id: "perform",
    href: "/",
  },
  {
    title: "Actors",
    id: "actors",
    href: "/actors",
  },
];

const App = () => {
  const location = useLocation();
  const currentHref = `/${location.pathname.split("/").slice(1)[0]}`;
  const currentTab = tabs.find((tab) => tab.href === currentHref);

  const navigate = useNavigate();

  const handleTabValueChange = (value: string) => {
    const newTab = tabs.find((tab) => tab.id === value);
    if (newTab && newTab.id !== currentTab?.id) {
      navigate(newTab.href);
    }
  };

  return (
    <main className="size-full flex flex-col">
      <Tabs
        className="w-full"
        value={currentTab?.id}
        onValueChange={handleTabValueChange}
      >
        <TabsList className="w-full">
          {tabs.map((tab) => (
            <TabsTrigger value={tab.id} key={tab.id}>
              {tab.title}
            </TabsTrigger>
          ))}
        </TabsList>
      </Tabs>
      <div className="w-full grow">
      <Routes>
        <Route path="/" element={<PerformPage />} />
        <Route path="/actors" element={<ActorsPage />} />
        </Routes>
      </div>
    </main>
  );
};

export default App;
