// import * as AlertDialog from '@radix-ui/react-alert-dialog';
import React, { FC, useContext, useRef, useState } from "react";

import {
  AlertDialog,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { Button, ButtonProps } from "@/components/ui/button";

type UseModalShowReturnType = {
  show: boolean;
  setShow: (show: boolean) => void;
  onHide: () => void;
};

export const useModalShow = (): UseModalShowReturnType => {
  const [show, setShow] = useState(false);

  const onHide = () => setShow(false);

  return { show, setShow, onHide };
};

type ConfirmationModalProps = {
  title: string | React.JSX.Element;
  message: string | React.JSX.Element;
  cancelButtonText?: string | React.JSX.Element;
  confirmButtonText?: string | React.JSX.Element;
  confirmButtonVariant?: ButtonProps["variant"];
};

type ModalContextType = {
  showConfirmation: (props: ConfirmationModalProps) => Promise<boolean>;
};

type ConfirmationModalContextProviderProps = {
  children: React.ReactNode;
};

const ConfirmationModalContext = React.createContext<ModalContextType>(
  {} as ModalContextType,
);

const ConfirmationModalContextProvider: FC<
  ConfirmationModalContextProviderProps
> = (props) => {
  const { show, setShow, onHide } = useModalShow();
  const [content, setContent] = useState<ConfirmationModalProps | undefined>();
  const resolver = useRef<(val: boolean | PromiseLike<boolean>) => void>(null);

  const handleShow = (props: ConfirmationModalProps): Promise<boolean> => {
    setContent(props);
    setShow(true);
    return new Promise((resolve) => (resolver.current = resolve));
  };

  const modalContext: ModalContextType = {
    showConfirmation: handleShow,
  };

  const handleSubmit = (positive: boolean) => {
    resolver.current?.(positive);
    onHide();
  };

  return (
    <ConfirmationModalContext.Provider value={modalContext}>
      {props.children}
      <AlertDialog open={show} onOpenChange={(open) => !open && onHide()}>
        {content && (
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>{content.title}</AlertDialogTitle>
              <AlertDialogDescription>{content.message}</AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel onClick={handleSubmit.bind(this, false)}>
                {content.cancelButtonText ?? "Cancel"}
              </AlertDialogCancel>
              <Button
                onClick={handleSubmit.bind(this, true)}
                variant={content.confirmButtonVariant}
              >
                {content.confirmButtonText ?? "Confirm"}
              </Button>
            </AlertDialogFooter>
          </AlertDialogContent>
        )}
      </AlertDialog>
    </ConfirmationModalContext.Provider>
  );
};

export const useConfirmationModalContext = (): ModalContextType =>
  useContext(ConfirmationModalContext);

export default ConfirmationModalContextProvider;
