#TODO write a description for this script
#@author 
#@category _NEW_
#@keybinding 
#@menupath 
#@toolbar 
from ghidra.app.util.headless import HeadlessScript

class Pre(HeadlessScript):
        def run(self):
            from ghidra.program.model.block import BasicBlockModel
            from ghidra.util.task import TaskMonitor

            bbm = BasicBlockModel(currentProgram)
            blocks = bbm.getCodeBlocks(TaskMonitor.DUMMY)
            block = blocks.next()

            with open('bbs.txt', 'w') as f:
                while block:
                    f.write(str(block.minAddress.offset) + '\n')
                    block = blocks.next()

Pre().run()
